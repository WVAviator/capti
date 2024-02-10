import mongoose from "mongoose";

export interface Recipe {
  userId: string;
  name: string;
  description: string;
  ingredients: string[];
  instructions: string[];
  imagePrompt: string;
  imageUrl: string;
  servings: number;
  time: number;
}

export type RecipeAttrs = Recipe;

export interface RecipeModel extends mongoose.Model<RecipeDocument> {
  build: (attrs: RecipeAttrs) => RecipeDocument;
}

export interface RecipeDocument extends RecipeAttrs, mongoose.Document {
  id: string;
}

const recipeSchema = new mongoose.Schema<RecipeAttrs>(
  {
    userId: {
      type: String,
      required: true,
    },
    name: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
    ingredients: {
      type: [String],
      required: true,
    },
    instructions: {
      type: [String],
      required: true,
    },
    imageUrl: {
      type: String,
      required: false,
    },
    servings: {
      type: Number,
      required: true,
    },
    time: {
      type: Number,
      required: true,
    },
  },
  {
    toJSON: {
      // Include virtual getters as properties (such as _id -> id) when converting to JSON
      virtuals: true,
      transform: (doc, ret, options) => {
        return ret;
      },
    },
    toObject: {
      // Include virtual getters as properties (such as _id -> id) when converting to objects
      virtuals: true,
      transform: (doc, ret, options) => {
        return ret;
      },
    },
  },
);

recipeSchema.statics.build = (attrs: RecipeAttrs) => {
  return new RecipeModel(attrs);
};

const RecipeModel = mongoose.model<RecipeDocument, RecipeModel>(
  "Recipe",
  recipeSchema,
);

export { RecipeModel };
