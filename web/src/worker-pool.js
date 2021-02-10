export class WorkerPool {
  constructor(workerPath) {
    let count = 8;

    if (navigator.hardwareConcurrency) {
      // Keep one core for the main thread
      count = navigator.hardwareConcurrency;
    }

    const boundOnMessage = this.onMessage.bind(this);
    this.pool = [];
    for (let i = 0; i < count; i++) {
      const worker = new Worker(workerPath, { name: `pool[${i}]` });
      worker.jobs = 0;
      this.pool.push(worker);
      worker.addEventListener("message", boundOnMessage);
    }

    this.msgId = 0;
    this.pending = [];
    this.sent = [];

    console.log(`Created ${count} workers`);
  }

  onMessage(event) {
    if (this.pending.length > 0) {
      this.runPendingJobs();
    }

    const sentMessages = this.sent;
    const sentMessagesCount = sentMessages.length;
    const id = event.data.id;

    // NOTE: Stupid for loop is significantly faster than array.find()
    // and we do not want to waste time until scheduling new jobs
    for (let i = 0; i < sentMessagesCount; i++) {
      const msg = sentMessages[i];
      if (msg.id === id) {
        msg.worker.jobs -= 1;

        if (event.data.error) {
          msg.reject(event.data.error);
        } else {
          msg.resolve(event.data.data);
        }

        return;
      }
    }
  }

  size() {
    return this.pool.length;
  }

  broadcast(type, data) {
    return Promise.all(this.pool.map(w => this.send(w, type, data)));
  }

  send(worker, type, data) {
    return new Promise((resolve, reject) => {
      this.sendInternal(worker, type, data, resolve, reject);
    });
  }

  sendInternal(worker, type, data, resolve, reject) {
    const id = this.msgId;
    let transfer = undefined;
    if (data && data.data && data.data.buffer) {
      transfer = [data.data.buffer];
    }
    worker.postMessage({ type, data, id }, transfer);
    this.sent.push({ worker, id, resolve, reject });
    this.msgId += 1;
    worker.jobs += 1;
  }

  runPendingJobs() {
    const workers = this.pool;
    const workersCount = workers.length;
    for (let i = 0; i < workersCount; i++) {
      let worker = workers[i];
      if (this.pending.length > 0) {
        if (worker.jobs < 1) {
          const job = this.pending.shift();
          this.sendInternal(
            worker,
            job.type,
            job.data,
            job.resolve,
            job.reject
          );
        }
      } else {
        return;
      }
    }

    if (this.pending.length > this.size()) {
      // Double schedule some workers so that they can continue right away
      for (let i = 0; i < workersCount; i++) {
        let worker = workers[i];
        if (this.pending.length > 0) {
          if (worker.jobs < 2) {
            const job = this.pending.shift();
            this.sendInternal(
              worker,
              job.type,
              job.data,
              job.resolve,
              job.reject
            );
          }
        } else {
          return;
        }
      }
    }
  }

  schedule(type, data) {
    let resolve;
    let reject;
    const promise = new Promise(function(_resolve, _reject) {
      resolve = _resolve;
      reject = _reject;
    });

    this.pending.push({ type, data, resolve, reject });

    this.runPendingJobs();

    return promise;
  }
}
