import express from "express";
import helloRouter from "./routes/hello";

const app = express();

app.use(helloRouter);

app.listen(3000, () => {
  console.log("Server is running on port 3000.");
});
