import { expect, test, describe, beforeAll, afterAll } from "bun:test";
import { startServer } from "./utils";
import { ChildProcess } from "child_process";

test("2 + 2", () => {
  expect(2 + 2).toBe(4);
});

describe("test group", () => {
    var server: ChildProcess;
    beforeAll(() => {
        console.log("beforeall")
        server = startServer();
    });

    test("2 + 2", () => {
        expect(2 + 2).toBe(4);
    });

    test("2 + 3", () => {
        expect(2 + 2).toBe(4);
    });

    afterAll(() => {
        console.log("after all")
        server.kill();
    });
});