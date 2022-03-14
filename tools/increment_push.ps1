#Publish an incremental push to github
function IncrementalPush() {
    #read in version from version file
    VERSION=$(cat version)
    Write-Output "Incremental push to github"
    git add .
    git commit -m "Incremental push to github version $VERSION"
    git push
    #update version file
    VERSION+=1
    Write-Output $VERSION > version
}