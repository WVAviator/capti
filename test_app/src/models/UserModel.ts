import mongoose from "mongoose";
import bcrypt from "bcrypt";

export interface User {
  email: string;
  displayName: string;
}
export interface UserAttrs extends User {
  password?: string;
}

export interface UserModel extends mongoose.Model<UserDocument> {
  build: (attrs: UserAttrs) => UserDocument;
}

export interface UserDocument extends UserAttrs, mongoose.Document {
  id: string;
  /**
   * Compare a provided password with the hashed password in the database
   * @param candidatePassword The password to compare
   * @returns True if the passwords match, false otherwise
   */
  comparePassword: (candidatePassword: string) => Promise<boolean>;
}

const userSchema = new mongoose.Schema<UserAttrs>(
  {
    email: {
      type: String,
      required: true,
    },
    displayName: {
      type: String,
      required: true,
    },
    password: {
      type: String,
      required: true,
    },
  },
  {
    toJSON: {
      // Include virtual getters as properties (such as _id -> id) when converting to JSON
      virtuals: true,
      transform: (doc, ret, options) => {
        delete ret.password;
        return ret;
      },
    },
    toObject: {
      // Include virtual getters as properties (such as _id -> id) when converting to objects
      virtuals: true,
      transform: (doc, ret, options) => {
        delete ret.password;
        return ret;
      },
    },
  },
);

userSchema.pre("save", async function save(next) {
  if (!this.password) return next();
  if (!this.isModified("password")) return next();
  try {
    const salt = await bcrypt.genSalt(10);
    this.password = await bcrypt.hash(this.password, salt);
    return next();
  } catch (err: any) {
    return next(err);
  }
});

userSchema.methods.comparePassword = async function (
  candidatePassword: string,
) {
  console.log(`Comparing ${candidatePassword} to hashed ${this.password}.`);
  return await bcrypt.compare(candidatePassword, this.password);
};

userSchema.statics.build = (attrs: UserAttrs): UserDocument => {
  return new UserModel(attrs);
};

const UserModel = mongoose.model<UserDocument, UserModel>("User", userSchema);

export { UserModel };
