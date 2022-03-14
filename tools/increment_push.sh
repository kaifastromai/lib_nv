#read in version from version file
version=`cat version`
#increment version
version=`expr $version + 1`
#write incremented version to version file
echo $version > version
#push version to github
git add .
git commit -m "Incremental push to github version $version"
git push 
