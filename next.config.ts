import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  reactStrictMode: true,

  experimental: {
    swcPlugins: [["./swc_plugin", {}]],
  },
};

export default nextConfig;
