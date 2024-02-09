import { Request, Response, Router } from "express";
import recipeController from "../controllers/recipeController";
import { authenticateUser } from "../controllers/authController";

const recipeRouter = Router();

recipeRouter.get(
  "/",
  authenticateUser,
  recipeController.getAllRecipes,
  (req: Request, res: Response) => {
    res.json(res.locals.recipes);
  },
);

recipeRouter.post(
  "/",
  authenticateUser,
  recipeController.createRecipe,
  (req: Request, res: Response) => {
    res.json(res.locals.recipe);
  },
);

recipeRouter.get(
  "/:id",
  authenticateUser,
  recipeController.getRecipeById,
  (req: Request, res: Response) => {
    res.json(res.locals.recipe);
  },
);

recipeRouter.delete(
  "/:id",
  authenticateUser,
  recipeController.deleteRecipeById,
  (req: Request, res: Response) => {
    res.json(res.locals.recipe);
  },
);

export default recipeRouter;
