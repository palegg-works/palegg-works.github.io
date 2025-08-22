#! /bin/sh
set -e

rm -rf ./docs
dx bundle --platform web --release
cp -rr ./target/dx/palegg-works-github-io/release/web/public ./docs

cd docs

# Add Simple Analytics
file="index.html"
snippet='<!-- 100% privacy-first analytics --><script data-collect-dnt="true" async src="https://scripts.simpleanalyticscdn.com/latest.js"></script><noscript><img src="https://queue.simpleanalyticscdn.com/noscript.gif?collect-dnt=true" alt="" referrerpolicy="no-referrer-when-downgrade"/></noscript>'
if sed --version >/dev/null 2>&1; then
  # GNU sed
  sed -i "/<\/body>/i $snippet" "$file"
else
  # BSD/macOS sed
  sed -i '' "/<\/body>/i\\
$snippet
" "$file"
fi

cd ..

echo Website source code updated!

