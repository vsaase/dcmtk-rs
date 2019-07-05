#![recursion_limit="128"]

#[macro_use]
extern crate cpp;

cpp!{{
    #include <stdio.h>
    #include <dcmtk/config/osconfig.h>
    #include <dcmtk/dcmdata/dctk.h>
    #include <dcmtk/dcmdata/dcistrmf.h>
}}

fn main() {
    unsafe {
        cpp!([] {
            OFCondition status;
            
            DcmMetaInfo metainfo;
            status = metainfo.loadFile("test.dcm");
            
            status.good();
            
            if (status.good())
            {
                OFString sopClassUID, xferUID;
                if (metainfo.findAndGetOFString(DCM_MediaStorageSOPClassUID, sopClassUID).good())
                    printf("SOP Class UID: %s\n", sopClassUID.c_str());
                if (metainfo.findAndGetOFString(DCM_TransferSyntaxUID, xferUID).good())
                    printf("Transfer Syntax UID: %s\n", xferUID.c_str());
                printf("Hello, DCMTK!\n");
            }
            
        });
    }
}