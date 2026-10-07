#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `FLTSL` reader - desc FLTSL"]
pub type FltslR = crate::FieldReader;
#[doc = "Field `FLTSL` writer - desc FLTSL"]
pub type FltslW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EDGSL` reader - desc EDGSL"]
pub type EdgslR = crate::FieldReader;
#[doc = "Field `EDGSL` writer - desc EDGSL"]
pub type EdgslW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IEN` reader - desc IEN"]
pub type IenR = crate::BitReader;
#[doc = "Field `IEN` writer - desc IEN"]
pub type IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVSEN` reader - desc CVSEN"]
pub type CvsenR = crate::BitReader;
#[doc = "Field `CVSEN` writer - desc CVSEN"]
pub type CvsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEN` reader - desc OUTEN"]
pub type OutenR = crate::BitReader;
#[doc = "Field `OUTEN` writer - desc OUTEN"]
pub type OutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - desc INV"]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - desc INV"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOE` reader - desc CMPOE"]
pub type CmpoeR = crate::BitReader;
#[doc = "Field `CMPOE` writer - desc CMPOE"]
pub type CmpoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPON` reader - desc CMPON"]
pub type CmponR = crate::BitReader;
#[doc = "Field `CMPON` writer - desc CMPON"]
pub type CmponW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc FLTSL"]
    #[inline(always)]
    pub fn fltsl(&self) -> FltslR {
        FltslR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 5:6 - desc EDGSL"]
    #[inline(always)]
    pub fn edgsl(&self) -> EdgslR {
        EdgslR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - desc IEN"]
    #[inline(always)]
    pub fn ien(&self) -> IenR {
        IenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CVSEN"]
    #[inline(always)]
    pub fn cvsen(&self) -> CvsenR {
        CvsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - desc OUTEN"]
    #[inline(always)]
    pub fn outen(&self) -> OutenR {
        OutenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc INV"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc CMPOE"]
    #[inline(always)]
    pub fn cmpoe(&self) -> CmpoeR {
        CmpoeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc CMPON"]
    #[inline(always)]
    pub fn cmpon(&self) -> CmponR {
        CmponR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc FLTSL"]
    #[inline(always)]
    pub fn fltsl(&mut self) -> FltslW<'_, CtrlSpec> {
        FltslW::new(self, 0)
    }
    #[doc = "Bits 5:6 - desc EDGSL"]
    #[inline(always)]
    pub fn edgsl(&mut self) -> EdgslW<'_, CtrlSpec> {
        EdgslW::new(self, 5)
    }
    #[doc = "Bit 7 - desc IEN"]
    #[inline(always)]
    pub fn ien(&mut self) -> IenW<'_, CtrlSpec> {
        IenW::new(self, 7)
    }
    #[doc = "Bit 8 - desc CVSEN"]
    #[inline(always)]
    pub fn cvsen(&mut self) -> CvsenW<'_, CtrlSpec> {
        CvsenW::new(self, 8)
    }
    #[doc = "Bit 12 - desc OUTEN"]
    #[inline(always)]
    pub fn outen(&mut self) -> OutenW<'_, CtrlSpec> {
        OutenW::new(self, 12)
    }
    #[doc = "Bit 13 - desc INV"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, CtrlSpec> {
        InvW::new(self, 13)
    }
    #[doc = "Bit 14 - desc CMPOE"]
    #[inline(always)]
    pub fn cmpoe(&mut self) -> CmpoeW<'_, CtrlSpec> {
        CmpoeW::new(self, 14)
    }
    #[doc = "Bit 15 - desc CMPON"]
    #[inline(always)]
    pub fn cmpon(&mut self) -> CmponW<'_, CtrlSpec> {
        CmponW::new(self, 15)
    }
}
#[doc = "desc CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
