goto_loc=$(goto-ally $LOCATION)
if [ -d $goto_loc ] ; then
	if [ -n "$goto_loc" ] ; then
		cd $goto_loc
	fi
fi
