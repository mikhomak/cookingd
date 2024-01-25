type Post = {
  title: String,
  shortText: String,
  text: String,
  main_image_url: string,
  rating: Number,
  likes: Number,
  createdAt: Date,
  tags: Tag[]
}

type Tag = {
  name: String
}
export type {Post, Tag}