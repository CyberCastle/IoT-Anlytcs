import { Configuration } from 'webpack';

// Webpack configuration to optimize angular build size
export default (config: Configuration) => {
   // Remove NgBuildAnalyticsPlugin
   config.plugins?.pop();

   return config;
};
