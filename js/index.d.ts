export type Permalink = {

  /**
   * Full URL encoded
   *
   * Note: It might be not exactly equal to input URL because of the trailing slash
   */
  href: string,

  /**
   * URL Origin
   */
  origin: string,

  /**
   * URL Pathname
   *
   * Note: It might be not exactly equal to input URL because of the trailing slash
   */
  pathname: string,

  /**
   * Country code
   *
   * It should be valid in terms of ISO 3166-1 (alpha 2)
   */
  country: string,

  /**
   * Language code
   *
   * It should be valid in terms of ISO 639-1
   */
  lang: string,

  /**
   * Name of the content type
   */
  contentType: string,

  /**
   * Title of the content, for SEO purpose
   */
  title: string | null,

  /**
   * Content identifier
   */
  id: string,

  /**
   * Optional data string.
   *
   * It (probably) is encoded via MessagePack + Base58 codec
   * @see https://github.com/daangn/urlpack/tree/main/packages/json
   */
  data: string | null,
};

export function parse(urlLike: string): Permalink;

export function normalize(permalink: Permalink): string;

export function canonicalize(permalink: Permalink, title: string): string;
