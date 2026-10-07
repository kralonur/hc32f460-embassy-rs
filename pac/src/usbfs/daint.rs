#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DaintSpec>;
#[doc = "Register `DAINT` writer"]
pub type W = crate::W<DaintSpec>;
#[doc = "Field `IEPINT` reader - desc IEPINT"]
pub type IepintR = crate::FieldReader;
#[doc = "Field `IEPINT` writer - desc IEPINT"]
pub type IepintW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OEPINT` reader - desc OEPINT"]
pub type OepintR = crate::FieldReader;
#[doc = "Field `OEPINT` writer - desc OEPINT"]
pub type OepintW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - desc IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        IepintR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - desc OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc IEPINT"]
    #[inline(always)]
    pub fn iepint(&mut self) -> IepintW<'_, DaintSpec> {
        IepintW::new(self, 0)
    }
    #[doc = "Bits 16:21 - desc OEPINT"]
    #[inline(always)]
    pub fn oepint(&mut self) -> OepintW<'_, DaintSpec> {
        OepintW::new(self, 16)
    }
}
#[doc = "desc DAINT\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaintSpec;
impl crate::RegisterSpec for DaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DaintSpec {}
#[doc = "`write(|w| ..)` method takes [`daint::W`](W) writer structure"]
impl crate::Writable for DaintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DaintSpec {}
