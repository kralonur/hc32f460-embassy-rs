#[doc = "Register `XTAL32NFR` reader"]
pub type R = crate::R<Xtal32nfrSpec>;
#[doc = "Register `XTAL32NFR` writer"]
pub type W = crate::W<Xtal32nfrSpec>;
#[doc = "Field `XTAL32NF` reader - desc XTAL32NF"]
pub type Xtal32nfR = crate::FieldReader;
#[doc = "Field `XTAL32NF` writer - desc XTAL32NF"]
pub type Xtal32nfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - desc XTAL32NF"]
    #[inline(always)]
    pub fn xtal32nf(&self) -> Xtal32nfR {
        Xtal32nfR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc XTAL32NF"]
    #[inline(always)]
    pub fn xtal32nf(&mut self) -> Xtal32nfW<'_, Xtal32nfrSpec> {
        Xtal32nfW::new(self, 0)
    }
}
#[doc = "desc XTAL32NFR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32nfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32nfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtal32nfrSpec;
impl crate::RegisterSpec for Xtal32nfrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xtal32nfr::R`](R) reader structure"]
impl crate::Readable for Xtal32nfrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal32nfr::W`](W) writer structure"]
impl crate::Writable for Xtal32nfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL32NFR to value 0"]
impl crate::Resettable for Xtal32nfrSpec {}
