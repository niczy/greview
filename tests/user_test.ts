import { afterAll, test, beforeAll, describe, expect } from "bun:test";
import { ChildProcess } from "child_process";
import { Tester, startServer } from "./utils";

describe("user API test", () => {
    var server: ChildProcess;
    var tester: Tester = new Tester();

    beforeAll(async () => {
        server = await startServer();
    });

    test("user APIs", async() => {
        const response = await tester.createUser({
            username: "username",
            password: "password",
        });
        expect(response.status).toBe(200);
        const createdUser = JSON.parse(await response.text());
        expect(createdUser["username"]).toEqual("username");
        expect(createdUser["verified"]).toBeFalse();
    });

    afterAll(() => {
        server.kill("SIGKILL");
    });
})