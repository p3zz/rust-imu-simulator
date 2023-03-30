let pitch = 0.0;
let roll = 0.0;
let yaw = 0.0;
setInterval(() => {
  process.stdout.write(`${pitch.toFixed(2)},${roll.toFixed(2)},${yaw.toFixed(2)}\n`);
  pitch += 0.1;
  roll += 0.1;
  yaw += 0.1;
}, 100);