#!/bin/sh

if [ -z "$USER_ALADIN" ]; then
    echo "export variable USER_ALADIN before launching script"
    exit 1
fi

DATEUPLOAD="$(date '+%Y-%m-%d')"

ssh $USER_ALADIN@aladin "sg hips -c 'mkdir -p /home/${USER_ALADIN}/al-tmp && rm -rf /home/${USER_ALADIN}/al-tmp/*'"
# Copy the dist files
scp dist/* $USER_ALADIN@aladin:~/al-tmp
# Copy the tar.gz
scp AladinLiteAssets.tar.gz $USER_ALADIN@aladin:~/al-tmp

ssh $USER_ALADIN@aladin "sg hips -c 'rm -rf /home/thomas.boch/AladinLite/www/api/v3/$DATEUPLOAD &&
mkdir -p /home/thomas.boch/AladinLite/www/api/v3/$DATEUPLOAD &&
cp /home/$USER_ALADIN/al-tmp/* /home/thomas.boch/AladinLite/www/api/v3/$DATEUPLOAD &&
rm -rf /home/thomas.boch/AladinLite/www/api/v3/latest &&
ln -s /home/thomas.boch/AladinLite/www/api/v3/$DATEUPLOAD /home/thomas.boch/AladinLite/www/api/v3/latest'"
