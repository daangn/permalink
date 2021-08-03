export type Permalink = {
  href: string,
  origin: string,
  pathname: string,
  country: string,
  lang: string,
  contentType: string,
  title: string | null,
  id: string,
  data: string | null,
};

export function parse(urlLike: string): Permalink;

export function normalize(permalink: Permalink): string;

export function canonicalize(permalink: Permalink, title: string): string;
