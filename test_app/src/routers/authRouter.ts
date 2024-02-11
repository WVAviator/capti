import { Request, Response, Router } from "express";
import {
  authenticateUser,
  signin,
  signup,
  deleteAccount,
} from "../controllers/authController";

const router = Router();

// Manual Auth Routes

router.post("/signup", signup, (req: Request, res: Response) => {
  res
    .status(200)
    .set({ "Content-Type": "application/json" })
    .json(res.locals.user);
});

router.post("/signin", signin, (req: Request, res: Response) => {
  res.status(200).json(res.locals.user);
});

// Other Routes

router.get("/user", authenticateUser, (req: Request, res: Response) => {
  return res.status(200).json(req.user);
});

router.post("/signout", authenticateUser, (req: Request, res: Response) => {
  req.session.userId = undefined;
  res.status(200).send();
});

router.delete(
  "/user",
  authenticateUser,
  deleteAccount,
  (req: Request, res: Response) => {
    res.status(200).send();
  },
);

export default router;
