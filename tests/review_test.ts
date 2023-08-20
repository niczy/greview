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
        const url = 'http://127.0.0.1:8085/_/review/create';
        const uid = "uid";
        const response = await fetch(url, {
            method: "POST",
            body: JSON.stringify({
                content: "Hello from greview!",
                guest_uid: uid 
            }),
            headers: { "Content-Type": "application/json" },
        });
        expect(response.status).toBe(200);
        const createdReview = JSON.parse(await response.text());
        console.log("created review: " + JSON.stringify(createdReview));

        const getReview = await fetch('http://127.0.0.1:8085/_/reviews', {
            method: "POST",
            body: JSON.stringify({
                guest_uid: uid 
            }),
            headers: { "Content-Type": "application/json" },
        })

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