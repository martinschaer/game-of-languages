type Action = "walk" | "run" | "end";

const bindings: Map<number, Action> = new Map([
  [1, "walk"],
  [2, "run"],
  [0, "end"],
]);

function loop() {
  let num = 0;
  loop:
  while (true) {
    console.log("1: Walk, 2: Run, 0: End");
    try {
      num = parseInt(prompt(">") || "");
    } catch (e) {
      num = -1;
    }
    const action = bindings.get(num);
    switch (action) {
      case "walk":
        console.log("Walking");
        break;
      case "run":
        console.log("Running");
        break;
      case "end":
        break loop;
      case undefined:
        console.log("Invalid action");
        break;
      default:
        const _exhaustiveCheck: never = action;
        console.log("Not implemented", _exhaustiveCheck);
        break;
    }
  }
}

loop();
