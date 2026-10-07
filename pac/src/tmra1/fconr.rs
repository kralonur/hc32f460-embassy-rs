#[doc = "Register `FCONR` reader"]
pub type R = crate::R<FconrSpec>;
#[doc = "Register `FCONR` writer"]
pub type W = crate::W<FconrSpec>;
#[doc = "Field `NOFIENTG` reader - desc NOFIENTG"]
pub type NofientgR = crate::BitReader;
#[doc = "Field `NOFIENTG` writer - desc NOFIENTG"]
pub type NofientgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKTG` reader - desc NOFICKTG"]
pub type NoficktgR = crate::FieldReader;
#[doc = "Field `NOFICKTG` writer - desc NOFICKTG"]
pub type NoficktgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOFIENCA` reader - desc NOFIENCA"]
pub type NofiencaR = crate::BitReader;
#[doc = "Field `NOFIENCA` writer - desc NOFIENCA"]
pub type NofiencaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKCA` reader - desc NOFICKCA"]
pub type NofickcaR = crate::FieldReader;
#[doc = "Field `NOFICKCA` writer - desc NOFICKCA"]
pub type NofickcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOFIENCB` reader - desc NOFIENCB"]
pub type NofiencbR = crate::BitReader;
#[doc = "Field `NOFIENCB` writer - desc NOFIENCB"]
pub type NofiencbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKCB` reader - desc NOFICKCB"]
pub type NofickcbR = crate::FieldReader;
#[doc = "Field `NOFICKCB` writer - desc NOFICKCB"]
pub type NofickcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc NOFIENTG"]
    #[inline(always)]
    pub fn nofientg(&self) -> NofientgR {
        NofientgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc NOFICKTG"]
    #[inline(always)]
    pub fn noficktg(&self) -> NoficktgR {
        NoficktgR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 8 - desc NOFIENCA"]
    #[inline(always)]
    pub fn nofienca(&self) -> NofiencaR {
        NofiencaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - desc NOFICKCA"]
    #[inline(always)]
    pub fn nofickca(&self) -> NofickcaR {
        NofickcaR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - desc NOFIENCB"]
    #[inline(always)]
    pub fn nofiencb(&self) -> NofiencbR {
        NofiencbR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - desc NOFICKCB"]
    #[inline(always)]
    pub fn nofickcb(&self) -> NofickcbR {
        NofickcbR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc NOFIENTG"]
    #[inline(always)]
    pub fn nofientg(&mut self) -> NofientgW<'_, FconrSpec> {
        NofientgW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc NOFICKTG"]
    #[inline(always)]
    pub fn noficktg(&mut self) -> NoficktgW<'_, FconrSpec> {
        NoficktgW::new(self, 1)
    }
    #[doc = "Bit 8 - desc NOFIENCA"]
    #[inline(always)]
    pub fn nofienca(&mut self) -> NofiencaW<'_, FconrSpec> {
        NofiencaW::new(self, 8)
    }
    #[doc = "Bits 9:10 - desc NOFICKCA"]
    #[inline(always)]
    pub fn nofickca(&mut self) -> NofickcaW<'_, FconrSpec> {
        NofickcaW::new(self, 9)
    }
    #[doc = "Bit 12 - desc NOFIENCB"]
    #[inline(always)]
    pub fn nofiencb(&mut self) -> NofiencbW<'_, FconrSpec> {
        NofiencbW::new(self, 12)
    }
    #[doc = "Bits 13:14 - desc NOFICKCB"]
    #[inline(always)]
    pub fn nofickcb(&mut self) -> NofickcbW<'_, FconrSpec> {
        NofickcbW::new(self, 13)
    }
}
#[doc = "desc FCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`fconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FconrSpec;
impl crate::RegisterSpec for FconrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fconr::R`](R) reader structure"]
impl crate::Readable for FconrSpec {}
#[doc = "`write(|w| ..)` method takes [`fconr::W`](W) writer structure"]
impl crate::Writable for FconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCONR to value 0"]
impl crate::Resettable for FconrSpec {}
