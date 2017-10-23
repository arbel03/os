ENTRY(start)

SECTIONS {

	.text :
	{
		*(mbr_header)
	}
}
