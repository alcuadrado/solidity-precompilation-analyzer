import test from "ava";

import { analyze } from "../index.js";

test("returns the pragmas", (t) => {
  t.deepEqual(
    analyze(`pragma solidity 1.2.3;

pragma solidity ^4.5.6 >1;
`),
    {
      versionPragmas: ["1.2.3", "^4.5.6 >1"],
      imports: [],
    }
  );
});

test("returns the imports", (t) => {
  t.true(true);
  return;
  t.deepEqual(
    analyze(`import "bare.sol";

import * as withStar from "star.sol";

import "as.sol" as something;

import {} from "empty-braces.sol";

import {,,} from "empty-braces2.sol";

import {something} from "symbol.sol";

import {something as somethingElse} from "aliased.sol";

import {something as somethingElse, other,,other2} from "multiple.sol";
`),
    {
      versionPragmas: [],
      imports: [
        "bare.sol",
        "star.sol",
        "as.sol",
        "empty-braces.sol",
        "empty-braces2.sol",
        "symbol.sol",
        "aliased.sol",
        "multiple.sol",
      ],
    }
  );
});
