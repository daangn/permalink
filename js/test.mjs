import { test } from 'zora';

import { parse, normalize, canonicalize } from './src/index.js';

const wellKnownOriginToCountry = {
  'https://daangn.com': 'kr',
  'https://www.daangn.com': 'kr',
  'https://ca.karrotmarket.com': 'ca',
  'https://jp.karrotmarket.com': 'jp',
  'https://uk.karrotmarket.com': 'uk',
  'https://us.karrotmarket.com': 'us',
  'https://kr.karrotmarket.com': 'kr',
};

const wellKnownCountryToLanguage = {
  'ca': 'en',
  'jp': 'ja',
  'uk': 'en',
  'us': 'en',
  'kr': 'ko',
};

test('parse', t => {
  t.throws(
    () => parse('https://www.daangn.com/kr/'),
    'invalid permlaink format',
  );

  t.equal(
    parse('https://www.daangn.com/kr/app/당근마켓-대한민국-1등-동네-앱-id1018769995/'),
    {
      country: 'kr',
      defaultLanguage: 'ko',
      serviceType: 'app',
      title: '당근마켓-대한민국-1등-동네-앱',
      id: 'id1018769995',
      data: null,
    },
    'full format should be valid',
  );

  t.equal(
    parse('https://www.daangn.com/kr/app/당근마켓-대한민국-1등-동네-앱-id1018769995/2LLSiWAs7A2dbJmq4nC9sCbGK1oTzLwZkwiqmji/'),
    {
      country: 'kr',
      defaultLanguage: 'ko',
      serviceType: 'app',
      title: '당근마켓-대한민국-1등-동네-앱',
      id: 'id1018769995',
      data: '2LLSiWAs7A2dbJmq4nC9sCbGK1oTzLwZkwiqmji',
    },
    'full format with data should be valid',
  );

  t.equal(
    parse('https://www.daangn.com/kr/app/당근마켓-대한민국-1등-동네-앱-id1018769995'),
    {
      country: 'kr',
      defaultLanguage: 'ko',
      serviceType: 'app',
      title: '당근마켓-대한민국-1등-동네-앱',
      id: 'id1018769995',
      data: null,
    },
    'skipping trailing slash would be fine',
  );

  t.equal(
    parse('https://www.daangn.com/KR/app/당근마켓-대한민국-1등-동네-앱-id1018769995/'),
    {
      country: 'kr',
      defaultLanguage: 'ko',
      serviceType: 'app',
      title: '당근마켓-대한민국-1등-동네-앱',
      id: 'id1018769995',
      data: null,
    },
    'country is case-insensitive',
  );

  t.equal(
    parse('https://www.daangn.com/kr/business-profiles/%EC%9E%84%EC%9D%80%ED%95%98%ED%91%B8%EB%93%9C-%EC%9D%B8%EC%B2%9C%EC%B0%BD%EA%B3%A0-97109917d5214963a7072732b61562df/'),
    {
      country: 'kr',
      defaultLanguage: 'ko',
      serviceType: 'business-profiles',
      title: '임은하푸드-인천창고',
      id: '97109917d5214963a7072732b61562df',
      data: null,
    },
    'real-world example 1',
  );

  for (const [origin, country] of Object.entries(wellKnownOriginToCountry)) {
    t.equal(
      parse(`${origin}/xx/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
      {
        country,
        defaultLanguage: wellKnownCountryToLanguage[country],
        serviceType: 'app',
        title: '당근마켓-대한민국-1등-동네-앱',
        id: 'id1018769995',
        data: null,
      },
      `country is overriden becuase ${origin} is well-known`,
    );
  }

  for (const [country, lang] of Object.entries(wellKnownCountryToLanguage)) {
    t.equal(
      parse(`http://localhost/${country}/app/당근마켓-대한민국-1등-동네-앱-id1018769995/`),
      {
        country,
        defaultLanguage: lang,
        serviceType: 'app',
        title: '당근마켓-대한민국-1등-동네-앱',
        id: 'id1018769995',
        data: null,
      },
      `language inferred becuase ${country} is well-known`,
    );
  }

  t.throws(
    () => parse('https://www.karrotmarket.com/zz/app/id1018769995/'),
    'it should throw if lang is not exist in the context',
  );

  t.equal(
    parse('https://www.daangn.com/kr/app/id1018769995/'),
    {
      country: 'kr',
      defaultLanguage: 'ko',
      serviceType: 'app',
      title: null,
      id: 'id1018769995',
      data: null,
    },
    'title might be skipped',
  );
});

test('normalize', t => {
  const permalink1 = parse('https://www.daangn.com/kr/app/당근마켓-대한민국-1등-동네-앱-id1018769995/');
  t.equal(
    normalize(permalink1),
    'https://www.karrotmarket.com/kr/app/id1018769995/',
  );

  const invalid = {
    country: null,
    defaultLanguage: null,
    serviceType: 'app',
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
  const permalink1 = parse('https://www.daangn.com/kr/app/id1018769995/');
  t.equal(
    canonicalize(permalink1, '당근마켓-대한민국-1등-동네-앱'),
    encodeURI('https://www.daangn.com/kr/app/당근마켓-대한민국-1등-동네-앱-id1018769995/'),
  );

  t.throws(
    () => canonicalize(permalink1),
    'it should throws if title is not bound explicitly',
  );

  const invalid = {
    country: null,
    defaultLanguage: null,
    serviceType: 'app',
    title: null,
    id: 'id1018769995',
    data: null,
  };
  t.throws(
    () => canonicalize(invalid, ''),
    'it should throw on invalid object',
  );
});
