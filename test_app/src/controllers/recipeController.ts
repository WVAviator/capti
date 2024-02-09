import { NextFunction, Request, Response } from "express";
import { RecipeModel } from "../models/RecipeModel";
import Joi from "joi";

const getAllRecipes = async (
  req: Request,
  res: Response,
  next: NextFunction,
) => {
  const { user } = req;

  try {
    const recipes = await RecipeModel.find({ userId: user.id });
    res.locals.recipes = recipes;
  } catch (error) {
    return next({
      log: `Error retrieving recipes from db: ${error}`,
      message: "Server Error",
      status: 500,
    });
  }

  return next();
};

const getRecipeById = async (
  req: Request,
  res: Response,
  next: NextFunction,
) => {
  const { id } = req.params;
  const { user } = req;

  try {
    const recipe = await RecipeModel.findOne({ _id: id, userId: user.id });

    if (!recipe) {
      return next({
        log: `Recipe not found: ${recipe}`,
        message: "Client Error",
        status: 404,
      });
    }

    res.locals.recipe = recipe;
  } catch (error) {
    return next({
      log: `Error retrieving recipe from db: ${error}`,
      message: "Server Error",
      status: 500,
    });
  }

  return next();
};

const deleteRecipeById = async (
  req: Request,
  res: Response,
  next: NextFunction,
) => {
  const { id } = req.params;
  const { user } = req;

  try {
    const recipe = await RecipeModel.findOneAndDelete({
      _id: id,
      userId: user.id,
    });

    if (!recipe) {
      return next({
        log: `Recipe not found: ${recipe}`,
        message: "Client Error",
        status: 404,
      });
    }

    res.locals.recipe = recipe;
  } catch (error) {
    return next({
      log: `Error deleting recipe from db: ${error}`,
      message: "Server Error",
      status: 500,
    });
  }

  return next();
};

export const createRecipeSchema = Joi.object({
  ingredients: Joi.array().items(Joi.string().required()).required(),
  instructions: Joi.array().items(Joi.string().required()).required(),
  name: Joi.string().required(),
  servings: Joi.number().required(),
  time: Joi.number().required(),
  description: Joi.string().required(),
});

const createRecipe = async (
  req: Request,
  res: Response,
  next: NextFunction,
) => {
  const { user } = req;

  if (!req.body) {
    return next({
      log: `Error retrieving request body`,
      message: "Server Error",
      status: 500,
    });
  }

  try {
    const recipe = createRecipeSchema.validate(req.body).value;

    const savedRecipe = RecipeModel.build({
      ...recipe,
      userId: user.id,
    });
    await savedRecipe.save();
    res.locals.recipe = savedRecipe;
    console.log("Receipe saved.");
  } catch (error) {
    return next({
      log: `Error saving recipe to db: ${error}`,
      message: "Server Error",
      status: 500,
    });
  }

  return next();
};

export default {
  getAllRecipes,
  getRecipeById,
  deleteRecipeById,
  createRecipe,
};
