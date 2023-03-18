/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: true,
  // Note: This feature is required to use NextJS Image in SSG mode.
  // Voir https://nextjs.org/docs/messages/export-image-api pour différentes solutions de contournement.
  images: {
    unoptimized: true,
  },
}

module.exports = nextConfig