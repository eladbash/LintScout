// @ts-ignore
const x: any = 1;
// @ts-nocheck
// @ts-expect-error
const y = x as string;
// eslint-disable-next-line
const z = 1;
