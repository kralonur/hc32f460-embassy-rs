#[doc = "Register `MCCR` reader"]
pub type R = crate::R<MccrSpec>;
#[doc = "Register `MCCR` writer"]
pub type W = crate::W<MccrSpec>;
#[doc = "Field `MDIVS` reader - desc MDIVS"]
pub type MdivsR = crate::FieldReader;
#[doc = "Field `MDIVS` writer - desc MDIVS"]
pub type MdivsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCKS` reader - desc MCKS"]
pub type McksR = crate::FieldReader;
#[doc = "Field `MCKS` writer - desc MCKS"]
pub type McksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - desc MDIVS"]
    #[inline(always)]
    pub fn mdivs(&self) -> MdivsR {
        MdivsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc MCKS"]
    #[inline(always)]
    pub fn mcks(&self) -> McksR {
        McksR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc MDIVS"]
    #[inline(always)]
    pub fn mdivs(&mut self) -> MdivsW<'_, MccrSpec> {
        MdivsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc MCKS"]
    #[inline(always)]
    pub fn mcks(&mut self) -> McksW<'_, MccrSpec> {
        McksW::new(self, 4)
    }
}
#[doc = "desc MCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`mccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MccrSpec;
impl crate::RegisterSpec for MccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mccr::R`](R) reader structure"]
impl crate::Readable for MccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mccr::W`](W) writer structure"]
impl crate::Writable for MccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCCR to value 0"]
impl crate::Resettable for MccrSpec {}
