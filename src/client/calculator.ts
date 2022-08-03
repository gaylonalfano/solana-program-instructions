import * as borsh from "borsh";
import * as math from "./math";

// 1. Compute the account space size (bytes) based off our data struct (Calculator)
// NOTE Our on-chain program is EXPECTING a schema that replicates/mimics
// our data account's struct (e.g., Calculator, Calculator, MathSquare, etc.)
// 1.1 Create a Class to mimic the struct
class Calculator {
  value = 0;
  constructor(fields: { value: number } | undefined = undefined) {
    if (fields) {
      this.value = fields.value;
    }
  }
}

// 1.2 Create a Map of the Class to convert into a struct
// NOTE This is for the ACCOUNT (not the INSTRUCTIONS)!
// For the account we use Map, for instructions we use BufferLayout
const CalculatorSchema = new Map([
  [Calculator, { kind: "struct", fields: [["value", "u32"]] }],
]);

// 1.3 Use Borsh to serialize and compute size in bytes (u8[].length)
// NOTE This is a serialized set of bytes that representation of our struct
const CALCULATOR_SIZE = borsh.serialize(
  CalculatorSchema,
  new Calculator()
).length;

async function main() {
  await math.runExample("calculator", CALCULATOR_SIZE);
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
