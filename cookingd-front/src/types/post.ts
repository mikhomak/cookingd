type Post = {
  id: String,
  title: String,
  shortText: String,
  text: String,
  mainImageUrl: string,
  rating: Number,
  likes: Number,
  createdAt: Date,
  tags: Tag[],
  user: {
    name: String
  }
}

type Tag = {
  name: String
}
export type {Post, Tag}