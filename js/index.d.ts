export type Permalink = {

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
  defaultLanguage: string,

  /**
   * Name of the content type
   */
  serviceType: string,

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
