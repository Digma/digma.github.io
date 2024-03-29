import { defineCollection, z } from "astro:content";

const blog = defineCollection({
  type: "content",
  // Type-check frontmatter using a schema
  schema: z.object({
    title: z.string(),
    description: z.string(),
    author: z.string(),
    // Transform string to Date object
    pubDate: z.coerce.date(),
    updatedDate: z.coerce.date().optional(),
    heroImage: z.object({
      url: z.string().optional(),
      alt: z.string().optional(),
    }),
    readingTime: z.number().optional(),
    canonicalUrl: z.string().optional(),
  }),
});

const products = defineCollection({
  type: "content",
  schema: z.object({
    title: z.string(),
    description: z.string(),
    author: z.string(),
    // Transform string to Date object
    pubDate: z.coerce.date(),
    updatedDate: z.coerce.date().optional(),
    heroImage: z.object({
      url: z.string().optional(),
      alt: z.string().optional(),
    }),
  }),
});

export const collections = { blog, products };
