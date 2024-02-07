import express from "express";

const router = express.Router();

router.get("/hello", (_req, res) => {
  res.status(200).json({ id: 1, message: "Hello, world!", completed: false });
});

export default router;
