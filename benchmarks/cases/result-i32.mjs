import { do_not_optimize } from "mitata";

export function createBenchmark({ call, implementation }) {
  const input = 42;

  function resultSlots(result) {
    if (!Array.isArray(result)) {
      throw new Error("unexpected benchmark function result");
    }

    if (implementation.kind === "js-bindgen") {
      if (
        result[0] !== undefined ||
        result[1] !== 0 ||
        result[2] !== input
      ) {
        throw new Error("unexpected benchmark function result");
      }

      return [result[1], result[2]];
    }

    if (
      result[0] !== input ||
      result[1] !== 0 ||
      result[2] !== 0
    ) {
      throw new Error("unexpected benchmark function result");
    }

    return [result[2], result[0]];
  }

  resultSlots(call(input));

  return function* () {
    let result;

    yield () => {
      result = call(input);
    };

    do_not_optimize(result);
  };
}
