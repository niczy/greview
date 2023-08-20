import { expect, test, describe, beforeAll, afterAll } from "bun:test";
import { startServer, Tester } from "./utils";
import { ChildProcess } from "child_process";

describe("test group", () => {
    var server: ChildProcess;
    var tester: Tester = new Tester();

    beforeAll(async () => {
        console.log("beforeall")
        server = await startServer();
    });

    test("post review", async () => {
        const uid = "uid";
        const response = await tester.postReview({
                content: "Hello from greview!",
                guest_uid: uid 
        });
        expect(response.status).toBe(200);
        const createdReview = JSON.parse(await response.text());
        console.log("created review: " + JSON.stringify(createdReview));

        const getReview = await tester.getReview({
                guest_uid: uid 
            });

        // console.log("get review: " + await getReview.text());
        expect(getReview.status).toBe(200);
        const readReview = JSON.parse(await getReview.text());
        const reviews: Array<Object> = readReview["reviews"];

        console.log("read review is " + JSON.stringify(reviews));
        
        expect(reviews.length).toBe(1);
        expect(createdReview.review).toEqual(reviews[0]);
    });

    afterAll(() => {
        console.log("after all")
        server.kill("SIGKILL");
    });
});