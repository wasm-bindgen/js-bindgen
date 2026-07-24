import { do_not_optimize } from "mitata";

export function createBenchmark({ call, input }) {
  const output = input.map(value => BigInt.asIntN(64, value));

  const expected = call(...input);

  if (
    !Array.isArray(expected) ||
    expected[0] !== output[0] ||
    expected[1] !== output[1]
  ) {
    throw new Error("unexpected benchmark function result");
  }

  return function* () {
    let result;

    yield () => {
      result = call(...input);
    };

    do_not_optimize(result);
  };
}
