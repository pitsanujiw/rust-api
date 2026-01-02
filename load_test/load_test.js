import http from 'k6/http';
import { check, sleep } from 'k6';
import { uuidv4 } from 'https://jslib.k6.io/k6-utils/1.4.0/index.js';

export const options = {
  stages: [
    { duration: '20s', target: 5 },   // warm-up
    { duration: '40s', target: 20 },  // ramp up
    { duration: '1m',  target: 50 },  // peak
    { duration: '20s', target: 0 },   // ramp down
  ],
  thresholds: {
    http_req_failed: ['rate<0.01'],      // error < 1%
    http_req_duration: ['p(95)<500'],    // p95 < 500ms
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

export default function () {
  // ---------- CREATE ----------
  const email = `load_${uuidv4()}@example.com`;
  let res = http.post(
    `${BASE_URL}/users`,
    JSON.stringify({
      username: 'loadtest',
      email: email,
      active: true,
    }),
    { headers: { 'Content-Type': 'application/json' } }
  );

  check(res, { 'create 201': (r) => r.status === 201 });
  const id = res.json('id');

  // ---------- FIND ONE ----------
  res = http.get(`${BASE_URL}/users/${id}`);
  check(res, { 'get 200': (r) => r.status === 200 });

  // ---------- FIND MANY ----------
  res = http.get(`${BASE_URL}/users?limit=20&offset=0`);
  check(res, { 'list 200': (r) => r.status === 200 });

  // ---------- UPDATE ----------
  res = http.put(
    `${BASE_URL}/users/${id}`,
    JSON.stringify({ active: false }),
    { headers: { 'Content-Type': 'application/json' } }
  );
  check(res, { 'update 200': (r) => r.status === 200 });

  // ---------- DELETE ----------
  res = http.del(`${BASE_URL}/users/${id}`);
  check(res, { 'delete 204': (r) => r.status === 204 });

  sleep(0.2);
}
