#[doc = "Register `VLTSEL` reader"]
pub type R = crate::R<VltselSpec>;
#[doc = "Register `VLTSEL` writer"]
pub type W = crate::W<VltselSpec>;
#[doc = "Field `RVSL` reader - desc RVSL"]
pub type RvslR = crate::FieldReader;
#[doc = "Field `RVSL` writer - desc RVSL"]
pub type RvslW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CVSL` reader - desc CVSL"]
pub type CvslR = crate::FieldReader;
#[doc = "Field `CVSL` writer - desc CVSL"]
pub type CvslW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C4SL` reader - desc C4SL"]
pub type C4slR = crate::FieldReader;
#[doc = "Field `C4SL` writer - desc C4SL"]
pub type C4slW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - desc RVSL"]
    #[inline(always)]
    pub fn rvsl(&self) -> RvslR {
        RvslR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc CVSL"]
    #[inline(always)]
    pub fn cvsl(&self) -> CvslR {
        CvslR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - desc C4SL"]
    #[inline(always)]
    pub fn c4sl(&self) -> C4slR {
        C4slR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc RVSL"]
    #[inline(always)]
    pub fn rvsl(&mut self) -> RvslW<'_, VltselSpec> {
        RvslW::new(self, 0)
    }
    #[doc = "Bits 8:11 - desc CVSL"]
    #[inline(always)]
    pub fn cvsl(&mut self) -> CvslW<'_, VltselSpec> {
        CvslW::new(self, 8)
    }
    #[doc = "Bits 12:14 - desc C4SL"]
    #[inline(always)]
    pub fn c4sl(&mut self) -> C4slW<'_, VltselSpec> {
        C4slW::new(self, 12)
    }
}
#[doc = "desc VLTSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`vltsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vltsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VltselSpec;
impl crate::RegisterSpec for VltselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`vltsel::R`](R) reader structure"]
impl crate::Readable for VltselSpec {}
#[doc = "`write(|w| ..)` method takes [`vltsel::W`](W) writer structure"]
impl crate::Writable for VltselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VLTSEL to value 0"]
impl crate::Resettable for VltselSpec {}
