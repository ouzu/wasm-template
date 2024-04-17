import init, { main, initThreadPool } from "../pkg";

import "./style.scss";

try {
  await init();
} catch (e) {
  // this happens in trace mode, when rayon is disabled
}

await initThreadPool(navigator.hardwareConcurrency);

main();
