import { test } from 'zora';

import { parse, normalize, canonicalize } from './src/index.js';

const wellKnownOriginToCountry = {
  'https://daangn.com': 'KR',
  'https://karrotmarket.com': undefined,
  'https://www.daangn.com': 'KR',
  'https://www.karrotmarket.com': undefined,
  'https://ca.karrotmarket.com': 'CA',
  'https://jp.karrotmarket.com': 'JP',
  'https://uk.karrotmarket.com': 'UK',
  'https://us.karrotmarket.com': 'US',
  'https://kr.karrotmarket.com': 'KR',
};

const wellKnownCountryToLanguage = {
  'CA': 'en',
  'JP': 'ja',
  'UK': 'en',
  'US': 'en',
  'KR': 'ko',
};

test('parse', t => {
  t.equal(
    parse('https://www.daangn.com/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/'),
    {
      href: encodeURI('https://www.daangn.com/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/'),
      origin: 'https://www.daangn.com',
      pathname: encodeURI('/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/'),
      country: 'KR',
      lang: 'ko',
      contentType: 'app',
      title: '당근마켓-대한민국-1등-동네-앱',
      id: 'id1018769995',
      data: null,
    },
    'full format should be valid',
  );

  t.equal(
    parse('https://www.daangn.com/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/2LLSiWAs7A2dbJmq4nC9sCbGK1oTzLwZkwiqmji/'),
    {
      href: encodeURI('https://www.daangn.com/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/2LLSiWAs7A2dbJmq4nC9sCbGK1oTzLwZkwiqmji/'),
      origin: 'https://www.daangn.com',
      pathname: encodeURI('/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/2LLSiWAs7A2dbJmq4nC9sCbGK1oTzLwZkwiqmji/'),
      country: 'KR',
      lang: 'ko',
      contentType: 'app',
      title: '당근마켓-대한민국-1등-동네-앱',
      id: 'id1018769995',
      data: '2LLSiWAs7A2dbJmq4nC9sCbGK1oTzLwZkwiqmji',
    },
    'full format with data should be valid',
  );

  t.equal(
    parse('https://www.daangn.com/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995'),
    {
      href: encodeURI('https://www.daangn.com/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/'),
      origin: 'https://www.daangn.com',
      pathname: encodeURI('/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/'),
      country: 'KR',
      lang: 'ko',
      contentType: 'app',
      title: '당근마켓-대한민국-1등-동네-앱',
      id: 'id1018769995',
      data: null,
    },
    'skipping trailing slash would be fine',
  );

  for (const [origin, country] of Object.entries(wellKnownOriginToCountry)) {
    if ([
      'https://karrotmarket.com',
      'https://www.karrotmarket.com',
    ].includes(origin)) {
      t.throws(
        () => parse(`${origin}/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
        'it should throw if country is not exist in the context',
      );
    } else {
      t.equal(
        parse(`${origin}/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
        {
          href: encodeURI(`${origin}/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
          origin,
          pathname: encodeURI(`/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
          country,
          lang: 'ko',
          contentType: 'app',
          title: '당근마켓-대한민국-1등-동네-앱',
          id: 'id1018769995',
          data: null,
        },
        `it can skip country because ${origin} is well-known`,
      );
    }
  }

  for (const [country, lang] of Object.entries(wellKnownCountryToLanguage)) {
    t.equal(
      parse(`https://www.daangn.com/${country}/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
      {
        href: encodeURI(`https://www.daangn.com/${country}/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
        origin: 'https://www.daangn.com',
        pathname: encodeURI(`/${country}/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
        country,
        lang,
        contentType: 'app',
        title: '당근마켓-대한민국-1등-동네-앱',
        id: 'id1018769995',
        data: null,
      },
      `it can skip lang because ${country} is well-known`,
    );
  }

  for (const [origin, country] of Object.entries(wellKnownOriginToCountry)) {
    if ([
      'https://karrotmarket.com',
      'https://www.karrotmarket.com',
    ].includes(origin)) {
      continue;
    }
    t.equal(
      parse(`${origin}/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
      {
        href: encodeURI(`${origin}/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
        origin,
        pathname: encodeURI(`/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
        country,
        lang: wellKnownCountryToLanguage[country],
        contentType: 'app',
        title: '당근마켓-대한민국-1등-동네-앱',
        id: 'id1018769995',
        data: null,
      },
      `it can even skip both country and lang because ${origin} is well-known`,
    );
  }

  t.throws(
    () => parse('https://www.karrotmarket.com/ZZ/app/id1018769995/'),
    'it should throw if lang is not exist in the context',
  );

  t.equal(
    parse('https://www.daangn.com/KR/ko/app/id1018769995/'),
    {
      href: encodeURI('https://www.daangn.com/KR/ko/app/id1018769995/'),
      origin: 'https://www.daangn.com',
      pathname: encodeURI('/KR/ko/app/id1018769995/'),
      country: 'KR',
      lang: 'ko',
      contentType: 'app',
      title: null,
      id: 'id1018769995',
      data: null,
    },
    'title might be skipped',
  );
});

test('normalize', t => {
  const permalink1 = parse('https://www.daangn.com/KR/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/');
  t.equal(
    normalize(permalink1),
    'https://www.karrotmarket.com/KR/app/id1018769995/',
  );

  const invalid = {
    country: null,
    lang: null,
    contentType: 'app',
    title: null,
    id: 'id1018769995',
    data: null,
  };
  t.throws(
    () => normalize(invalid),
    'it should throw on invalid object',
  );
});

test('canonicalize', t => {
  const permalink1 = parse('https://www.daangn.com/KR/app/id1018769995/');
  t.equal(
    canonicalize(permalink1, '당근마켓-대한민국-1등-동네-앱'),
    encodeURI('https://www.daangn.com/ko/app/당근마켓-대한민국-1등-동네-앱-id1018769995/'),
  );

  t.throws(
    () => canonicalize(permalink1),
    'it should throws if title is not bound explicitly',
  );

  const invalid = {
    country: null,
    lang: null,
    contentType: 'app',
    title: null,
    id: 'id1018769995',
    data: null,
  };
  t.throws(
    () => canonicalize(invalid, ''),
    'it should throw on invalid object',
  );
});
