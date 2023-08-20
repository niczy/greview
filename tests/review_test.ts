import { expect, test, describe, beforeAll, afterAll } from "bun:test";
import { startServer } from "./utils";
import { ChildProcess } from "child_process";

describe("test group", () => {
    var server: ChildProcess;
    beforeAll(async () => {
        console.log("beforeall")
        server = await startServer();
    });

    test("post review", async () => {
        expect(2 + 2).toBe(4);
        const url = 'http://127.0.0.1:8085/_/review/create';
        const response = await fetch(url, {
            method: "POST",
            body: JSON.stringify({ content: "Hello from Bun!" }),
            headers: { "Content-Type": "application/json" },
        });
        console.log(await response.text());
    });

    test("2 + 3", () => {
        expect(2 + 2).toBe(4);
    });

    afterAll(() => {
        console.log("after all")
        server.kill();
    });
});