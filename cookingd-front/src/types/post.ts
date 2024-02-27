type Post = {
  title: String,
  shortText: String,
  text: String,
  mainImageUrl: string,
  rating: Number,
  likes: Number,
  createdAt: Date,
  tags: Tag[]
}

type Tag = {
  name: String
}
export type {Post, Tag}