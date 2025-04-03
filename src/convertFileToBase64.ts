import { EncodedImage } from "./types.ts";

export const convertFileToBase64 = (file: File): Promise<EncodedImage> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onloadend = () => {
      if (typeof reader.result === "string") {
        const encodedImage = reader.result.split(",")[1] as EncodedImage;
        resolve(encodedImage); // Remove the data URL prefix
      } else {
        reject(new Error("Failed to convert file to base64"));
      }
    };
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
};
