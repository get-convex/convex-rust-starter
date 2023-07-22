import { v } from "convex/values";
import { mutation, query } from "./_generated/server";

export const add = mutation({
  args: { author: v.string(), body: v.string() },
  handler: async (ctx, { author, body }) => {
    await ctx.db.insert("messages", { author, body });
  },
});

export const collect = query({
  handler:async (ctx) => {
  return await ctx.db.query("messages").collect();
}
});
