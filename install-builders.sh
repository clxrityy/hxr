echo "Installing builders..."
touch build.sh
echo "#!/bin/bash" > build.sh
echo "echo 'Building...'" >> build.sh
echo "cargo install --path ." >> build.sh
echo "echo 'Done!'" >> build.sh
chmod +x build.sh
touch builder.txt
echo """
@@ This installs all the development scripts.
@@ To run them you just pass them in as a file in your shell.


--------

@ chmod +x ./install-builders.sh
@ ./install-builders.sh 
! ~~    Installs all the building scripts 
!   ~~   You can delete this after they've been installed locally 


----------


@@@ SCRIPTS (AFTER INSTALLATION)


@ ./build.sh -- bundles the cargo package








there's not many scripts.. that's all i need rn
""" > builder.txt
echo "Done!"
echo "Now you can run ./build.sh to build the cargo package."
echo "You can also read the builder.txt file for more information."
echo "Deleting install-builders.sh..."
rm -rf install-builders.sh