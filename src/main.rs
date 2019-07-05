#![recursion_limit="128"]

#[macro_use]
extern crate cpp;

cpp!{{
    #include <dcmtk/config/osconfig.h>
    #include <dcmtk/dcmdata/dctk.h>
    #include <dcmtk/dcmdata/dcistrmf.h>
}}

fn main() {
    unsafe {
        cpp!([] {
            OFCondition status;

            /* approach 1 */
            DcmMetaInfo metainfo;
            status = metainfo.loadFile("test.dcm");
            if (status.good())
            {
                OFString sopClassUID, xferUID;
                if (metainfo.findAndGetOFString(DCM_MediaStorageSOPClassUID, sopClassUID).good())
                    COUT << "SOP Class UID: " << sopClassUID << OFendl;
                if (metainfo.findAndGetOFString(DCM_TransferSyntaxUID, xferUID).good())
                    COUT << "Transfer Syntax UID: " << xferUID << OFendl;
                metainfo.print(COUT);
            }
        });
    }
}