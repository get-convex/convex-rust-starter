import { v } from "convex/values";
import { mutation, query } from "./_generated/server";

export const add = mutation({
  args: {
    author: v.string(),
    body: v.string(),
  },
  handler: async ({ db }, { author, body }) => {
    await db.insert("messages", {
      author,
      body,
    });
  },
});

export const collect = query(async ({ db }) => {
  return await db.query("messages").collect();
});
