#[doc = "Register `DAINTMSK` reader"]
pub type R = crate::R<DaintmskSpec>;
#[doc = "Register `DAINTMSK` writer"]
pub type W = crate::W<DaintmskSpec>;
#[doc = "Field `IEPINTM` reader - desc IEPINTM"]
pub type IepintmR = crate::FieldReader;
#[doc = "Field `IEPINTM` writer - desc IEPINTM"]
pub type IepintmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OEPINTM` reader - desc OEPINTM"]
pub type OepintmR = crate::FieldReader;
#[doc = "Field `OEPINTM` writer - desc OEPINTM"]
pub type OepintmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - desc IEPINTM"]
    #[inline(always)]
    pub fn iepintm(&self) -> IepintmR {
        IepintmR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - desc OEPINTM"]
    #[inline(always)]
    pub fn oepintm(&self) -> OepintmR {
        OepintmR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc IEPINTM"]
    #[inline(always)]
    pub fn iepintm(&mut self) -> IepintmW<'_, DaintmskSpec> {
        IepintmW::new(self, 0)
    }
    #[doc = "Bits 16:21 - desc OEPINTM"]
    #[inline(always)]
    pub fn oepintm(&mut self) -> OepintmW<'_, DaintmskSpec> {
        OepintmW::new(self, 16)
    }
}
#[doc = "desc DAINTMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaintmskSpec;
impl crate::RegisterSpec for DaintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daintmsk::R`](R) reader structure"]
impl crate::Readable for DaintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure"]
impl crate::Writable for DaintmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DaintmskSpec {}
