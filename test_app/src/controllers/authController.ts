import { NextFunction, Request, Response } from "express";
import {
  UserModel as User,
  UserDocument,
  UserModel,
} from "../models/UserModel";

/**
 * This middleware function will log a user in with the provided email and password. It will then set the user's id on the session and set the user on res.locals.user. It will then call next() to allow the request to continue. If the email or password are not provided, this function will throw an error. If the email is not found in the database, or if the password does not match the password in the database, this function will throw an error.
 * @param req
 * @param res
 * @param next
 * @returns
 */
export const signin = async (
  req: Request,
  res: Response,
  next: NextFunction,
) => {
  try {
    const { email, password } = req.body;
    if (!email || !password) {
      return next({
        log: `Request body missing email or password, req.body: ${req.body}`,
        message: "Missing email or password",
        status: 400,
      });
    }

    const user = (await User.findOne({ email })) as UserDocument;
    if (!user) {
      return next({
        log: "Invalid email",
        message: "Invalid email or password",
        status: 400,
      });
    }

    const userAuthenticated = await user.comparePassword(password);
    if (!userAuthenticated) {
      return next({
        log: "Invalid password",
        message: "Invalid email or password",
        status: 400,
      });
    }

    req.session.userId = user.id;
    res.locals.user = user;

    return next();
  } catch (error) {
    return next({
      log: `Error connecting to db: ${error}`,
      message: "Server Error",
      status: 500,
    });
  }
};

/**
 * This middleware function will register a new user with the provided email and password. It will hash the password before saving it to the database. It will then set the user's id on the session and set the user on res.locals.user. It will then call next() to allow the request to continue.
 * @param req
 * @param res
 * @param next
 * @returns
 */
export const signup = async (
  req: Request,
  res: Response,
  next: NextFunction,
) => {
  try {
    const { email, displayName, password } = req.body;
    if (!email || !password || !displayName) {
      return next({
        log: `Request body missing email, displayName, or password, req.body: ${req.body}`,
        message: "Missing email, display name, or password",
        status: 400,
      });
    }

    const user = new User({
      email,
      displayName,
      password,
    });

    await user.save();

    res.locals.user = user;
    req.session.userId = user.id;

    return next();
  } catch (error) {
    return next({
      log: `Error connecting to db: ${error}`,
      message: "Server Error",
      status: 500,
    });
  }
};

/**
 * This middleware function closes the gap between OAuth and traditional login. If the user is not using OAuth, the the req.user will be undefined. The req.user will then be populated with the user's information by using the userID property on req.session. If there is no req.session.userID or there is not matching user in the database, this function will not error. It will simply call next() and allow the request to continue. Use this middleware with each request before using any other middleware that accesses user data on the req.user property.
 * @param req
 * @param res
 * @param next
 * @returns
 */
export const authenticateUser = async (
  req: Request,
  res: Response,
  next: NextFunction,
) => {
  try {
    const { userId } = req.session;
    if (!userId) {
      return next({
        log: "Attempted use of endpoint without authentication",
        message: "Not authorized",
        status: 401,
      });
    }
    const user = await User.findById(userId);
    if (!user) {
      return next({
        log: "Attempted use of endpoint without authentication",
        message: "Not authorized",
        status: 401,
      });
    }
    req.user = user as UserDocument;
    return next();
  } catch (error) {
    return next({
      log: `Error connecting to db: ${error}`,
      message: "Server Error",
      status: 500,
    });
  }
};
