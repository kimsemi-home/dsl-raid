export async function readJsonFile<T>(input: HTMLInputElement): Promise<T | undefined> {
  const file = input.files?.[0];
  if (!file) {
    return undefined;
  }
  return JSON.parse(await file.text()) as T;
}
