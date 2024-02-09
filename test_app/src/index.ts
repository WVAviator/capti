import express, { NextFunction, Request, Response } from "express";
import dotenv from "dotenv";
import mongoose from "mongoose";
import cookieParser from "cookie-parser";
import session from "express-session";
import MongoStore from "connect-mongo";
import { ServerError } from "./types/server-error";
import authRouter from "./routers/authRouter";
import recipeRouter from "./routers/recipeRouter";

dotenv.config();

const app = express();

app.use(cookieParser());
app.use(express.json());
console.log("Mongo URI: ", process.env.MONGODB_URI);
app.use(
  session({
    secret: "j3J77g20opPsX121",
    resave: false,
    saveUninitialized: false,
    store: MongoStore.create({ mongoUrl: process.env.MONGODB_URI || "" }),
  }),
);

app.use("/auth", authRouter);
app.use("/recipes", recipeRouter);

app.use((err: ServerError, req: Request, res: Response, next: NextFunction) => {
  console.log("Server error occurred.");
  err.log && console.error(err.log);

  res.status(err.status || 500).json({
    message: err.message || "Something went wrong.",
  });
});

const start = async () => {
  try {
    await mongoose.connect(process.env.MONGODB_URI || "");
    console.log("Connected to MongoDB");
  } catch (err) {
    console.log("Error connecting to MongoDB");
    console.error(err);
  }

  const port = process.env.PORT;

  app.listen(port, () => {
    console.log(`Listening on ${port}`);
  });
};

start();
