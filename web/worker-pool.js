export class WorkerPool {
  constructor(workerPath) {
    let count = 6;

    if (navigator.hardwareConcurrency) {
      // if we leave some corse for the OS and the browser, we get more time in our workers
      count = Math.max(navigator.hardwareConcurrency - 2, 1);
    }

    this.pool = [];
    for (var i = 0; i < count; i++) {
      const worker = new Worker(workerPath);
      worker.jobs = 0;
      this.pool.push(worker);
    }

    this.msgId = 0;
    this.pending = [];

    console.log(`Created ${count} workers`);
  }

  size() {
    return this.pool.length;
  }

  broadcast(type, data) {
    return Promise.all(this.pool.map(w => this.send(w, type, data)));
  }

  send(worker, type, data) {
    this.msgId += 1;
    const messageId = this.msgId;
    let transfer = [];
    if (data && data.data && data.data.buffer) {
      transfer.push(data.data.buffer);
    }
    worker.jobs += 1;
    worker.postMessage({ type, data, id: messageId }, transfer);

    return new Promise((resolve, reject) => {
      const onMessage = event => {
        const { type, id, data, error } = event.data;

        if (id === messageId) {
          worker.removeEventListener("message", onMessage);
          worker.jobs -= 1;
          this.runPendingJobs();

          if (error) {
            reject(error);
          } else {
            resolve(data);
          }
        }
      };

      worker.addEventListener("message", onMessage);
    });
  }

  runPendingJobs() {
    this.pool.forEach(worker => {
      if (worker.jobs === 0 && this.pending.length > 0) {
        const job = this.pending.pop();
        this.send(worker, job.type, job.data).then(job.resolve, job.reject);
      }
    });
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
